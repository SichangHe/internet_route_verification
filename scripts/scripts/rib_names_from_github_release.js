// Script to run in the browser to extract CSV file names and
// links from GitHub Release and remove their entries.
function csvAnchorsWith(pattern, filePrefix) {
    const anchors = document.querySelectorAll(`a[href*="${pattern}"]`)
    filePrefix = filePrefix || "";
    const formattedOutput = Array.from(anchors).map(anchor => {
        const href = anchor.href;
        const filename = anchor.innerText.trim();
        anchor.parentNode.parentNode.remove();
        return `CsvFile("${filePrefix}${filename}", "${href}")`;
    }).join(", ");
    return formattedOutput;
}

const docstring = "From <https://github.com/SichangHe/internet_route_verification/releases/tag/raw-data>."

const ribs_listings = csvAnchorsWith(`0000.`, `../data/ribs/`);
console.log(`ribs = [${ribs_listings}]\n"""${docstring}"""`);
