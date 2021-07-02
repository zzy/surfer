$(document).ready(function () {
    var isSelected = "is-selected";

    var navGlobalArticles = $(".nav-global-articles");
    var navGlobalCategories = $(".nav-global-categories");
    var navGlobalTopics = $(".nav-global-topics");
    var navSignSignin = $(".nav-sign-signin");
    var navSignRegister = $(".nav-sign-register");
    var logo = $(".js-logo");

    navGlobalArticles.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navSelectedClean();
        navGlobalArticles.toggleClass(isSelected, true);
    });

    navGlobalCategories.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navSelectedClean();
        navGlobalCategories.toggleClass(isSelected, true);
    });

    navGlobalTopics.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navSelectedClean();
        navGlobalTopics.toggleClass(isSelected, true);
    });

    navSignSignin.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navSelectedClean();
        navSignSignin.toggleClass(isSelected, true);
    });

    navSignRegister.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navSelectedClean();
        navSignRegister.toggleClass(isSelected, true);
    });

    logo.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navSelectedClean();
    });

    function navSelectedClean() {
        navGlobalArticles.toggleClass(isSelected, false);
        navGlobalCategories.toggleClass(isSelected, false);
        navGlobalTopics.toggleClass(isSelected, false);
        navSignSignin.toggleClass(isSelected, false);
        navSignRegister.toggleClass(isSelected, false);
    }
});
