function changeThemeTo(val) {
    document.documentElement.setAttribute("data-theme", val);
    // save prefs in the browser
    localStorage.setItem("blog-rust-lang-org-theme", val);
    // close the theme dropdown
    document.getElementById("theme-choice").classList.toggle("display");
}

function dropdown () {
    document.getElementById("theme-choice").classList.toggle("display");
}

// Check for saved user preference on load, else check user agent prefs
const savedTheme = localStorage.getItem("blog-rust-lang-org-theme");
if (savedTheme) {
    document.documentElement.setAttribute("data-theme", savedTheme);
} else if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
    document.documentElement.setAttribute("data-theme", "dark");
}
