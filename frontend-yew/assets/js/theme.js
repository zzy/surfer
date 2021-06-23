(function () {
    var forceSetting = localStorage.getItem("forceDarkModeOn");
    var browserPrefersDark = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
    var darkModeEnabled = forceSetting === "true" || (!forceSetting && browserPrefersDark);

    if (browserPrefersDark) {
        document.body.classList.toggle("theme-system", darkModeEnabled);
        document.body.classList.toggle("theme-dark", false);
    }
    else {
        document.body.classList.toggle("theme-system", true);
        document.body.classList.toggle("theme-dark", darkModeEnabled);
    }
}());
