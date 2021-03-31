class KeywordTags extends HTMLElement {

    constructor() {
        super()

        var shadow = this.shadow = this.attachShadow({ mode: 'open' })

        var tagsTemplate = this.tagsTemplate = document.getElementById('keyword-tags-template')
        shadow.appendChild(tagsTemplate.content.cloneNode(true))

        var keywordTagsContainer = this.keywordTagsContainer = shadow.querySelector(".keyword-tags")
        var tagTemplate = this.tagTemplate = document.getElementById('keyword-tag-template')

        var tagValues = this.getAttribute('tag-values')
        tagValues = tagValues.split(',')
        tagValues.forEach((tagValue) => {
            tagValue = tagValue.trim()
            if (tagValue.length != 0)
                this.addTag(tagValue)
        })

        var input = this.input = shadow.querySelector("input")
        input.addEventListener("keydown", (e) => {
            if (e.key === "Enter" || e.key === ",") {
                e.preventDefault()
                this.addTag(input.value)
                input.value = ''
            }
            if (e.key === "Backspace" && input.value === '') {
                this.removeLastTag()
            }
        })

    }

    addTag(tagValue) {
        var tagFrag = this.tagTemplate.content.cloneNode(true)
        var keywordNode = tagFrag.querySelector(".keyword")
        keywordNode.textContent = tagValue

        this.keywordTagsContainer.appendChild(
            tagFrag
        )

        var tagNode = this.getLastTag()
        tagNode.addEventListener("click", (e) => {
            this.removeTag(tagNode)
        })
    }

    removeTag(tagNode) {
        this.keywordTagsContainer.removeChild(tagNode)
    }

    removeLastTag() {
        var lastTag = this.getLastTag()
        if (lastTag !== null)
            this.removeTag(lastTag)
    }

    getLastTag() {
        var tagNodes = this.keywordTagsContainer.querySelectorAll('.keyword-tag')
        return tagNodes.length ? tagNodes[tagNodes.length - 1] : null
    }

    connectedCallback() {
        // TODO
    }

}

customElements.define('keyword-tags', KeywordTags)
