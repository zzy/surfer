"use strict";

sequential("/js/library.jquery.js",
    function () {
        random("/js/theme.js?132689068675031052");
        random("/js/hamburger.js?132689068675031052");
        random("/js/navigation.js?132689068675031052");
        random("/js/nav.selected.js?132689068675031052");
        random("/js/search.js?132689068675031052");
        random("/js/stacks.min.js?132689068675031052");
        random("/js/feature.darkmode.js?132689068675031052");
        // These can be removed
        random("//static.ruonou.com/js/bdtj-bh.js");
        random("//static.ruonou.com/js/bdts.js");
    }
);

function random(src) {
    let script = document.createElement("script");
    script.src = src;

    script.onload = script.onreadystatechange = function () {
        if (!this.readyState || this.readyState === "loaded" || this.readyState === "complete") {
            this.onload = this.onreadystatechange = null;
            this.parentNode.removeChild(this);
        }
    }

    document.body.appendChild(script);
};

function sequential(src, success) {
    let script = document.createElement("script");
    script.src = src;

    success = success || function () { };
    script.onload = script.onreadystatechange = function () {
        if (!this.readyState || this.readyState === "loaded" || this.readyState === "complete") {
            success();

            this.onload = this.onreadystatechange = null;
            this.parentNode.removeChild(this);
        }
    }

    document.body.appendChild(script);
}
