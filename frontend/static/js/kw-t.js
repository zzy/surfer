class KeywordTags extends HTMLElement {

    constructor() {
        super()

        var shadow = this.shadow = this.attachShadow({ mode: 'open' })

        var tagsTemplate = this.tagsTemplate = document.getElementById('keyword-tags-template')
        shadow.appendChild(tagsTemplate.content.cloneNode(true))

        var keywordTagsContainer = this.keywordTagsContainer = shadow.querySelector(".keyword-tags")
        var tagTemplate = this.tagTemplate = document.getElementById('keyword-tag-template')

        var input = this.input = shadow.querySelector("input")
        input.addEventListener("keydown", (e) => {
            var inputValue = input.value.trim();

            if (e.key === " " || e.key === "Enter" || e.key === ",") {
                e.preventDefault();
                if (inputValue !== '')
                    this.addTag(inputValue)
                input.value = ''
            }
            if (e.key === "Backspace" && inputValue === '') {
                this.removeLastTag()
            }
        })
    }

    addTag(tagValue) {
        var tagFrag = this.tagTemplate.content.cloneNode(true)
        var keywordNode = tagFrag.querySelector(".keyword")
        keywordNode.textContent = tagValue.trim()

        this.keywordTagsContainer.appendChild(
            tagFrag
        )

        var tagNode = this.getLastTag()
        tagNode.addEventListener("click", (e) => {
            this.removeTag(tagNode)
        })

        this.setTopics()
    }

    removeTag(tagNode) {
        this.keywordTagsContainer.removeChild(tagNode)

        this.setTopics()
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

    setTopics() {
        var topic_names = []

        var tagNodes = this.keywordTagsContainer.querySelectorAll('.keyword-tag')
        tagNodes.forEach((tagNode) => {
            topic_names.push(tagNode.textContent.trim())
        })

        document.getElementById("topic_names").value = topic_names.toString()
    }

    connectedCallback() {
        var tagValues = this.getAttribute('tag-values')
        tagValues = tagValues.split(',')

        tagValues.forEach((tagValue) => {
            tagValue = tagValue.trim()
            if (tagValue !== '')
                this.addTag(tagValue)
        })
    }

}

customElements.define('keyword-tags', KeywordTags)
