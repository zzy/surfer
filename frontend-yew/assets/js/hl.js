let script = document.createElement("script");
script.src = "/js/highlight.min.js?132689068675031052";

script.onload = script.onreadystatechange = function () {
    if (!this.readyState || this.readyState === "loaded" || this.readyState === "complete") {
        hljs.highlightAll();

        this.onload = this.onreadystatechange = null;
        this.parentNode.removeChild(this);
    }
}

document.body.appendChild(script);
