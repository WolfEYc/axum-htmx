const copyButtonLabel = "Copy";

// use a class selector if available
let blocks = document.querySelectorAll('copyblock');

blocks.forEach((block) => {
    // only add button if browser supports Clipboard API
    if (!navigator.clipboard) return;
    
    let button = document.createElement("button");
    button.innerText = copyButtonLabel;
    block.appendChild(button);
    
    button.addEventListener("click", async () => {
        await copyCode(block);
    });
});

async function copyCode(block, button) {
    let code = block.querySelector("input");
    let text = code.innerText;

    await navigator.clipboard.writeText(text);

    button.innerText = "Copied!";
    setTimeout(() => {
        button.innerText = copyButtonLabel;
    }, 700);
}