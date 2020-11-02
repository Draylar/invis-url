const ZERO = '​';
const ONE = '‍';
const SPACE = '‌';

// Elements
const input = document.getElementById("primary-input");
const url_info = document.getElementById("primary-url-information");
const URL_CHARACTERS = document.getElementById("url-characters");
const CHANGE_PERCENTAGE = document.getElementById("change-percentage");
const interact = document.getElementById("primary-interact");
const result = document.getElementById("primary-result")
const copy = document.getElementById("primary-copy");
var visible = false;

// -------------- INPUT UPDATING -------------- //

input.addEventListener('input', event => {
    var val = url_to_invisible(event.target.value);
    const inputLength = input.value.length;
    const invisibleLength = val.length;

    if (val.length > 0 && !visible) {
        show();
        visible = true;
    } else if (val.length == 0 && visible) {
        hide();
        visible = false;
    }

    URL_CHARACTERS.innerText = "URL Characters: " + val.length;
    CHANGE_PERCENTAGE.innerText = "Change in Length: " + (invisibleLength / inputLength).toFixed(1) + "x";
    result.value = val;
});

// -------------- TOGGLING OUTPUT VISIBILITY -------------- //

function hide() {
    url_info.classList.remove("active");

    setTimeout(() => {
        url_info.style.display = 'none';

        setTimeout(() => {
            interact.classList.remove("active");
        }, 200);
    }, 350);
}

function show() {
    interact.classList.add("active");

    setTimeout(() => {
        url_info.style.display = 'block'; // make url info exist
        
        // fade opacity on url info
        setTimeout(() => {
            url_info.classList.add("active");
        }, 200);
    }, 200);
}

// -------------- COPY URL -------------- //

copy.addEventListener('click', event => {
    result.select();
    result.setSelectionRange(0, 999999);
    document.execCommand("copy");
});

// -------------- CONVERSION UTILITY FUNCTIONS -------------- //

function url_to_invisible(url) {
    if (url.length == 0) {
        return url;
    }

    let binary = string_to_binary(url);
    let replacements = binary_to_replacements(binary);
    return "https://invis-url.draylar.dev/" + replacements;
}

function string_to_binary(string) {
    return string.split('').map(char => {
        return char.charCodeAt(0).toString(2);
    }).join(' ');
}

function binary_to_replacements(binary) {
    let invis = binary;
    invis = invis.replaceAll("0", ZERO);
    invis = invis.replaceAll("1", ONE);
    invis = invis.replaceAll(" ", SPACE);
    return invis;
}


// -------------- URL INPUT DYNAMIC DOTS -------------- //
var stage = 0;

function update() {
    setTimeout(() => {
        input.placeholder = "Start typing any URL here" + ".".repeat(stage);
        stage++;

        // Reset stage
        if (stage >= 4) {
            stage = 0;
        }

        // Repeat
        update();
    }, 1000);
}

update();

//  -------------- CHANGING LABEL -------------- //
const label = document.getElementById("primary-title");
const endings = [];
var index = 0;

// Setup default title endings
endings.push({ text: "spooky", color: "#fcba03" });
endings.push({ text: "funny", color: "#3aa5e8" });
endings.push({ text: "disguised", color: "#240945" });
endings.push({ text: "invisible", color: "#b8b8b8" }); // default

function updateTitle() {
    setTimeout(() => {
        // Update title
        label.innerHTML = "Make your link " + "<span style=\"text-decoration: underline; color: " + endings[index].color + ";\">" + endings[index].text + "</span>" + ".";

        // Cycle to next
        index++;
        if(index == 4) {
            index = 0;
        }

        // Repeat
        updateTitle();
    }, 6000);
}

// updateTitle();