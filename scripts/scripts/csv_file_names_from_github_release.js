// Script to run in the browser to extract CSV file names and links from GitHub Release.
function csvAnchorsWith(pattern, filePrefix) {
    const anchors = document.querySelectorAll(`a[href*="${pattern}"]`)
    filePrefix = filePrefix || "";
    const formattedOutput = Array.from(anchors).map(anchor => {
        const href = anchor.href;
        const filename = href.substring(href.lastIndexOf("/") + 1);
        return `CsvFile("${filePrefix}${filename}", "${href}")`;
    }).join(", ");
    return formattedOutput;
}

const version = 5;
const docstring = "From <https://github.com/SichangHe/internet_route_verification/releases/tag/data-193>."
const stats_names = ["as_stats", "as_pair_stats", "route_stats", "route_first_hop_stats"];

const csv_files = stats_names.map(stats_name => {
    const listing = csvAnchorsWith(`--${stats_name}${version}.csv`, `all${version}/`)
    return `${stats_name}_all = [${listing}]\n"""${docstring}"""`;
}).join("\n\n");

console.log(csv_files);
