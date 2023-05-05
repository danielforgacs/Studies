let create_button = document.getElementById("create-button")
console.log("::create button: ", create_button)
create_button.addEventListener("click", post_alert)

function post_alert() {
    let title_input = document.getElementById("name");
    console.log("::title_input: ", title_input)
    alert(title_input.value)
    title_input.value = null
}


function renderItems(items, processType, elementID, processFunction) {
    let placeholder = "<div>"
    let itemsMeta = []
    for (i = 0; i < items.length; i++) {
        let title = items[i]["title"]
        let placeholderId = processType + "-" + title.replaceAll(" ", "-")
        placeholder += "<div>" + title + "<button " + 'id="' + placeholderId + '">' + processType + '</button>' + "</div>"
        itemsMeta.push({"id": placeholderId, "title": title})
    }
    placeholder += "</div>"
    document.getElementById(elementID).innerHTML = placeholder
    for (i = 0; i < itemsMeta.length; i++) {
        document.getElementById(itemsMeta[i]["id"]).addEventListener("click", processFunction)
    }
}