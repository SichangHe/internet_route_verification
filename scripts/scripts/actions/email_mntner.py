"""Run at `scripts/` as `python3 -m scripts.actions.email_mntner`.

Email maintainers of ASes with many `spec_import_customer` or `spec_export_customers`.
"""

import json
import os
import smtplib
import subprocess
from dataclasses import dataclass
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from typing import DefaultDict

from dotenv import load_dotenv
from rpsl_lexer.lines import expressions, lines_continued, rpsl_objects

from scripts.csv_files import relaxed_filter_as_info

relaxed_filter_tech_c_emails_path = "relaxed_filter_tech_c_emails.csv"
relaxed_filter_tech_c_wo_emails_path = "relaxed_filter_tech_c_wo_emails.json"
SPECIAL_WHOIS_ADDRESS = {
    "ARIN": "rr.arin.net",
    "LEVEL3": "rr.Level3.net",
    "NTTCOM": "rr.ntt.net",
    "JPIRR": "jpirr.nic.ad.jp",
    "TC": "whois.bgp.net.br",
}


@dataclass
class AsInfo:
    export_peer_asns: list[int]
    export_as_any: bool
    import_customer_asns: list[int]
    tech_c: str
    source: str


def deserialize_json(file_path: str):
    with open(file_path, "r") as f:
        data = json.load(f)
    return {int(k): AsInfo(**v) for k, v in data.items()}


def query_whois(tech_c: str, source: str):
    address = SPECIAL_WHOIS_ADDRESS.get(source)
    if address is None:
        address = f"whois.{source.lower()}.net"
    return subprocess.check_output(["whois", "-h", address, tech_c], text=True)


def extract_emails(whois_output: str):
    emails: list[str] = []
    non_skipped_lines = (
        line for line in whois_output.splitlines() if not line.startswith("%")
    )
    for obj in rpsl_objects(lines_continued(non_skipped_lines)):
        for key, expr in expressions(obj.body.splitlines()):
            if key == "e-mail":
                emails.append(expr.strip())
    return emails


A_FEW = 3
ONE = 1
HEADER = """<p>Hi teams, in the (simplified) RPSL below:</p>

"""
FOOTER = """
<p>We are a team developing a tool to analyze routing policies expressed in the RPSL,<sup><a href="fn1">[1]</a></sup> and this feedback would allow us to better understand the intended semantics in generating reports.</p>

<ol>
<li id="fn1"><a href="https://github.com/SichangHe/internet_route_verification">Internet route verification</a>.</li>
</ol>
"""


def compose_email(tech_c: str, as_infos: list[tuple[int, AsInfo]]):
    export_self_cases: list[str] = []
    import_customer_cases: list[str] = []
    peer = ""
    customer = ""
    export_asn = 0
    import_asn = 0
    for asn, info in as_infos:
        if info.export_as_any:
            export_self_cases.append(f"""aut-num: AS{asn}
export: to AS-ANY announce AS{asn}
tech-c: {tech_c}
""")
            peer = "-ANY"
            export_asn = asn

        elif len(info.export_peer_asns) > 0:
            export_lines = "\n".join(
                f"export: to AS{peer} announce AS{asn}"
                for peer in info.export_peer_asns[:A_FEW]
            )
            if peer == "":
                peer = f"{info.export_peer_asns[0]}"
            if export_asn == 0:
                export_asn = asn
            if len(info.export_peer_asns) > A_FEW:
                export_lines += "\n# Omitting similar lines..."
            export_self_cases.append(f"""aut-num: AS{asn}
{export_lines}
tech-c: {tech_c}
""")

        if len(info.import_customer_asns) > 0:
            import_lines = "\n".join(
                f"import: from AS{customer} accept AS{customer}"
                for customer in info.import_customer_asns[:A_FEW]
            )
            if customer == "":
                customer = f"{info.import_customer_asns[0]}"
            if import_asn == 0:
                import_asn = asn
            if len(info.import_customer_asns) > A_FEW:
                import_lines += "\n# Omitting similar lines..."
            import_customer_cases.append(f"""aut-num: AS{asn}
{import_lines}
tech-c: {tech_c}
""")

    export_self_body = "\n".join(export_self_cases[:ONE])
    if len(export_self_cases) > ONE:
        export_self_body += "\n# Omitting similar aut-nums...\n"
    if export_self_body != "":
        assert export_asn != 0
        export_self_body = f"""<pre><code>{export_self_body}</code></pre>

<p>Do you mean that over the BGP session with (e.g.) AS{peer}:</p>

<ol>
<li>AS{export_asn} will export any routes originated by AS{export_asn} or its customers; or</li>
<li>AS{export_asn} will export only routes originated by AS{export_asn} itself; or</li>
<li>Something else?</li>
</ol>
"""

    import_customer_body = "\n".join(import_customer_cases[:ONE])
    if len(import_customer_cases) > ONE:
        import_customer_body += "\n# Omitting similar aut-nums...\n"
    if import_customer_body != "":
        assert import_asn != 0
        import_customer_body = f"""<pre><code>{import_customer_body}</code></pre>

<p>Do you mean that when processing routes received from (e.g.) AS{customer}:</p>

<ol>
<li>AS{import_asn} will import any route received from AS{customer} (including AS{customer}'s customers); or</li>
<li>AS{import_asn} will only import routes originated by AS{customer} itself; or</li>
<li>Something else?</li>
</ol>
"""

    separation = ""
    if export_self_body != "" and import_customer_body != "":
        separation = """
<hr>
<p>Similarly, in:</p>

        """

    return f"""{HEADER}{export_self_body}{separation}{import_customer_body}{FOOTER}"""


SMTP_SERVER = "smtp.gmail.com"
SMTP_PORT = 587
SUBJECT = "Inquiry: What do you mean by these RPSL lines in your aut-num?"


def send_gmails(
    smtp_user: str,
    smtp_password: str,
    from_email: str,
    cc_email: str,
    email_msgs: list[tuple[str, str]],
):
    with smtplib.SMTP(SMTP_SERVER, SMTP_PORT) as server:
        server.starttls()
        server.login(smtp_user, smtp_password)

        for to_email, msg_body in email_msgs:
            msg = MIMEMultipart()
            msg["From"] = from_email
            msg["To"] = to_email
            msg["Cc"] = f"{from_email},{cc_email}"
            msg["Subject"] = SUBJECT
            msg.attach(MIMEText(msg_body, "html"))

            server.sendmail(
                from_email, [to_email, from_email, cc_email], msg.as_string()
            )
            print(f"Sent Gmail to {to_email}.")


def main():
    relaxed_filter_as_info.download_if_missing()
    as_infos = deserialize_json(relaxed_filter_as_info.path)
    tech_c_map: DefaultDict[str, list[tuple[int, AsInfo]]] = DefaultDict(lambda: list())
    for asn, info in as_infos.items():
        tech_c_map[info.tech_c].append((asn, info))

    if relaxed_filter_tech_c_emails_path not in os.listdir():
        tech_c_outputs: dict[str, str] = {}
        for tech_c, asns in tech_c_map.items():
            output = query_whois(tech_c, asns[0][1].source)
            tech_c_outputs[tech_c] = output

        tech_c_emails: dict[str, str] = {}
        tech_c_w_multi_emails: dict[str, list[str]] = {}
        tech_c_wo_email: set[str] = set()
        for tech_c, output in tech_c_outputs.items():
            emails = extract_emails(output)
            match len(emails):
                case 0:
                    tech_c_wo_email.add(tech_c)
                case 1:
                    tech_c_emails[tech_c] = emails[0]
                case _:
                    print(f"Multiple emails found for {tech_c}: {emails}")
                    tech_c_w_multi_emails[tech_c] = emails

        with open(relaxed_filter_tech_c_emails_path, "a") as f:
            f.write(
                "tech_c,email\n"
                + "".join(
                    f"{tech_c},{email}\n" for tech_c, email in tech_c_emails.items()
                )
            )
        tech_c_wo_emails_map = {
            tech_c: {"asns": [n for n, _ in infos], "source": infos[0][1].source}
            for tech_c, infos in tech_c_map.items()
            if tech_c in tech_c_wo_email
        }
        with open(relaxed_filter_tech_c_wo_emails_path, "w") as f:
            json.dump(tech_c_wo_emails_map, f, indent=2)
    else:
        with open(relaxed_filter_tech_c_emails_path, "r") as f:
            tech_c_emails = {
                line.split(",")[0]: line.split(",")[1].strip()
                for line in f.read().splitlines()[1:]
            }

    source_email_counts: DefaultDict[str, int] = DefaultDict(lambda: 0)
    for tech_c in tech_c_emails:
        source_email_counts[tech_c_map[tech_c][0][1].source] += 1

    email_msgs: list[tuple[str, str]] = []
    for tech_c, email in tech_c_emails.items():
        msg = compose_email(tech_c, tech_c_map[tech_c])
        email_msgs.append((email, msg))

    load_dotenv()
    smtp_user = os.getenv("SMTP_USER")
    assert smtp_user is not None
    smtp_password = os.getenv("SMTP_PASSWD")
    assert smtp_password is not None
    from_email = os.getenv("FROM_EMAIL")
    assert from_email is not None
    cc_email = os.getenv("CC_EMAIL")
    assert cc_email is not None
    send_gmails(smtp_user, smtp_password, from_email, cc_email, email_msgs)


main() if __name__ == "__main__" else None
