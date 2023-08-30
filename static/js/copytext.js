const StandbyLabel = "Copy";
const PressedLabel = "Copied!"

function copyToClipboard(text, btn) {
    if (!navigator.clipboard) return;
    copyCode(text, btn)
}

async function copyCode(text, btn) {
    btn.disabled = true;
    await navigator.clipboard.writeText(text);

    setTimeout(() => {
        btn.disabled = false;
    }, 700);
}