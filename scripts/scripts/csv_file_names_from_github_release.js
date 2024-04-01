// Script to run in the browser to extract CSV file names and links from GitHub Release.
function csvAnchorsWith(pattern, filePrefix) {
    const anchors = document.querySelectorAll(`a[href*="${pattern}"]`)
    filePrefix = filePrefix || "";
    const formattedOutput = Array.from(anchors).map(anchor => {
        const href = anchor.getAttribute("href");
        const filename = href.substring(href.lastIndexOf("/") + 1);
        return `CsvFile("${filePrefix}${filename}", "${href}")`;
    }).join(", ");
    return formattedOutput;
}

// AS stats:
console.log(csvAnchorsWith("--as_stats2.csv", "all2/"));
// AS pair stats:
console.log(csvAnchorsWith("--as_pair_stats2.csv", "all2/"));
// Route stats:
console.log(csvAnchorsWith("--route_stats2.csv", "all2/"));
// Route first-hop stats:
console.log(csvAnchorsWith("--route_first_hop_stats2.csv", "all2/"));
