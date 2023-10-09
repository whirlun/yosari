import { LitElement,html } from "lit";
import { customElement, property } from "lit/decorators";
import cover from "./IMG_0004.jpeg";
@customElement("yo-cviewer")
class ComicViewer extends LitElement {
    @property({type: Boolean})
    visible = false

    constructor() {
        super();
        this.visible = false;
    }

    protected createRenderRoot(){
        return this;
    }

    render() {
        console.log(this.visible);
        return html`
            <div class="absolute z-10 w-full h-screen bg-gray-200 ${this.visible === true ? "visible" : "invisible"}">
                <img src="${cover}" />
            </div>
        `
    }
}