$(document).ready(function () {
    var isSelected = "is-selected";

    var navGlobalArticles = $(".nav-global-articles");
    var navGlobalCategories = $(".nav-global-categories");
    var navGlobalTopics = $(".nav-global-topics");
    var navGlobalExplore = $(".nav-global-explore");
    var navSignSignin = $(".nav-sign-signin");
    var navSignRegister = $(".nav-sign-register");
    var logo = $(".js-logo");

    navGlobalArticles.click(function (e) {
        navSelectedClean();
        navGlobalArticles.toggleClass(isSelected, true);
    });

    navGlobalCategories.click(function (e) {
        navSelectedClean();
        navGlobalCategories.toggleClass(isSelected, true);
    });

    navGlobalTopics.click(function (e) {
        navSelectedClean();
        navGlobalTopics.toggleClass(isSelected, true);
    });

    navGlobalExplore.click(function (e) {
        navSelectedClean();
        navGlobalExplore.toggleClass(isSelected, true);
    });

    navSignSignin.click(function (e) {
        navSelectedClean();
        navSignSignin.toggleClass(isSelected, true);
    });

    navSignRegister.click(function (e) {
        navSelectedClean();
        navSignRegister.toggleClass(isSelected, true);
    });

    logo.click(function (e) {
        navSelectedClean();
    });

    function navSelectedClean() {
        navGlobalArticles.toggleClass(isSelected, false);
        navGlobalCategories.toggleClass(isSelected, false);
        navGlobalTopics.toggleClass(isSelected, false);
        navGlobalExplore.toggleClass(isSelected, false);
        navSignSignin.toggleClass(isSelected, false);
        navSignRegister.toggleClass(isSelected, false);
    }
});
