$(document).ready(function () {
    var isSelected = "is-selected";

    var navGlobalArticles = $(".nav-global-articles");
    var navGlobalCategories = $(".nav-global-categories");
    var navGlobalTopics = $(".nav-global-topics");
    var logo = $(".js-logo");

    navGlobalArticles.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navGlobalClean();
        navGlobalArticles.toggleClass(isSelected, true);
    });

    navGlobalCategories.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navGlobalClean();
        navGlobalCategories.toggleClass(isSelected, true);
    });

    navGlobalTopics.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navGlobalClean();
        navGlobalTopics.toggleClass(isSelected, true);
    });

    logo.click(function (e) {
        e.preventDefault();
        e.stopPropagation();

        navGlobalClean();
    });

    function navGlobalClean() {
        navGlobalArticles.toggleClass(isSelected, false);
        navGlobalCategories.toggleClass(isSelected, false);
        navGlobalTopics.toggleClass(isSelected, false);
    }
});
