import "./app.css";

import {LitElement, css, html} from "lit";
import {property, customElement} from 'lit/decorators';

export @customElement("yo-home") class Home extends LitElement {
    @property
    name = "";
    static styles = css`:host {color: blue}`;
    constructor() {
        super();
        this.name = "World";
    }

    render() {
        return html`<div class="flex flex-row">
            <div class="drawer">
                <div class="drawer-content">
                </div>
            </div>
            <div class="drawer-side">
                
            </div>
        </div>`
    }
}
