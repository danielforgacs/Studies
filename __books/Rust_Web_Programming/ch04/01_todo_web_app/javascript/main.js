let create_button = document.getElementById("create-button")
console.log("::create button: ", create_button)
create_button.addEventListener("click", post_alert)

function post_alert() {
    let title_input = document.getElementById("name");
    console.log("::title_input: ", title_input)
    alert(title_input.value)
    title_input.value = null
}
