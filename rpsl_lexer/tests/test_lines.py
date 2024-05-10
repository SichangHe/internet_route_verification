from rpsl_lexer.lines import lines_continued

RAW_LINES_EXAMPLES = [
    """aut-num:        AS1880
as-name:        STUPI
org:            ORG-STPA1-RIPE
remarks:        STUPI-NET Stockholm Core
import:         from AS1755
                action pref=100;
                accept AS1755 AS1653 AS1257 AS1883
import:         from AS1883
                action pref=200;
                accept AS1755 AS1653 AS1257 AS1883
import:         from AS1881
                action pref=100;
                accept AS1881
import:         from AS1882
                action pref=100;
                accept AS1882
export:         to AS1755
                announce AS1880 AS1881 AS1882 AS1883
export:         to AS1883
                announce ANY
export:         to AS1881
                announce ANY
export:         to AS1882
                announce ANY
default:        to AS1755
                action pref=100;
                networks AS1755 AS690
default:        to AS1883
                action pref=200;
                networks AS1755 AS690
admin-c:        DUMY-RIPE
tech-c:         DUMY-RIPE
status:         ASSIGNED
notify:         roll@stupi.se
mnt-by:         RIPE-NCC-END-MNT
mnt-by:         STUPI-MNT
created:        1970-01-01T00:00:00Z
last-modified:  2017-11-15T09:12:33Z
source:         RIPE
sponsoring-org: ORG-SA58-RIPE
remarks:        ****************************
remarks:        * THIS OBJECT IS MODIFIED
remarks:        * Please note that all data that is generally regarded as personal
remarks:        * data has been removed from this object.
remarks:        * To view the original object, please query the RIPE Database at:
remarks:        * http://www.ripe.net/whois
remarks:        ****************************
"""
]

CONTINUED_RAW_LINES_EXAMPLES = [
    [
        "aut-num:        AS1880",
        "as-name:        STUPI",
        "org:            ORG-STPA1-RIPE",
        "remarks:        STUPI-NET Stockholm Core",
        "import:         from AS1755 action pref=100; accept AS1755 AS1653 AS1257 AS1883",
        "import:         from AS1883 action pref=200; accept AS1755 AS1653 AS1257 AS1883",
        "import:         from AS1881 action pref=100; accept AS1881",
        "import:         from AS1882 action pref=100; accept AS1882",
        "export:         to AS1755 announce AS1880 AS1881 AS1882 AS1883",
        "export:         to AS1883 announce ANY",
        "export:         to AS1881 announce ANY",
        "export:         to AS1882 announce ANY",
        "default:        to AS1755 action pref=100; networks AS1755 AS690",
        "default:        to AS1883 action pref=200; networks AS1755 AS690",
        "admin-c:        DUMY-RIPE",
        "tech-c:         DUMY-RIPE",
        "status:         ASSIGNED",
        "notify:         roll@stupi.se",
        "mnt-by:         RIPE-NCC-END-MNT",
        "mnt-by:         STUPI-MNT",
        "created:        1970-01-01T00:00:00Z",
        "last-modified:  2017-11-15T09:12:33Z",
        "source:         RIPE",
        "sponsoring-org: ORG-SA58-RIPE",
        "remarks:        ****************************",
        "remarks:        * THIS OBJECT IS MODIFIED",
        "remarks:        * Please note that all data that is generally regarded as personal",
        "remarks:        * data has been removed from this object.",
        "remarks:        * To view the original object, please query the RIPE Database at:",
        "remarks:        * http://www.ripe.net/whois",
        "remarks:        ****************************",
    ]
]


def test_lines_continued():
    for example, expected in zip(RAW_LINES_EXAMPLES, CONTINUED_RAW_LINES_EXAMPLES):
        result = lines_continued(example.splitlines())
        assert list(result) == expected
