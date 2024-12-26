"use strict";

function changeThemeTo(val) {
    if (val === "system") {
        document.documentElement.removeAttribute("data-theme");
        // delete explicit theme pref from browser storage
        if (storageAvailable("localStorage")) {
            localStorage.removeItem("blog-rust-lang-org-theme");
        }
    } else {
        document.documentElement.setAttribute("data-theme", val);
        // save theme prefs in the browser
        if (storageAvailable("localStorage")) {
            localStorage.setItem("blog-rust-lang-org-theme", val);
        }
    }
    // the theme dropdown will close itself when returning to the dropdown() caller
}

function dropdown () {
    document.getElementById("theme-choice").classList.toggle("showThemeDropdown");
}

// courtesy MDN
function storageAvailable(type) {
    let storage;
    try {
        storage = window[type];
        const x = "__storage_test__";
        storage.setItem(x, x);
        storage.removeItem(x);
        return true;
    } catch (e) {
        return (
            e instanceof DOMException &&
                e.name === "QuotaExceededError" &&
                // acknowledge QuotaExceededError only if there's something already stored
            storage &&
                storage.length !== 0
        );
    }
}

function handleBlur(event) {
    const parent = document.getElementById("theme-choice");
    if (!parent.contains(document.activeElement) &&
        !parent.contains(event.relatedTarget) &&
        parent.classList.contains("showThemeDropdown")
       ) {
        console.debug('Closing the dropdown');
        parent.classList.remove("showThemeDropdown");
    }
}

// close the theme dropdown if clicking somewhere else
document.querySelector('.theme-icon').onblur = handleBlur;

// Check for saved user preference on load, else check and save user agent prefs
if (storageAvailable("localStorage")) {
    const savedTheme = localStorage.getItem("blog-rust-lang-org-theme");
    if (savedTheme) {
        document.documentElement.setAttribute("data-theme", savedTheme);
    }
}

// show the theme selector only if JavaScript is enabled/available
document.querySelector('.theme-icon').style.display = 'block';
