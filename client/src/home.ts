import "./app.css";
import cover from "./IMG_0004.jpeg";

import { LitElement, css, html } from "lit";
import { property, customElement, state } from "lit/decorators";
import "./navbar";
import "./cover";
import "./mplayer";
import "./cviewer";
export
@customElement("yo-home")
class Home extends LitElement {
  @property({type: String})
  name = "";
  static styles = css`
    :host {
      color: blue;
    }
  `;
  @state()
  cviewerVisibility: Boolean = false;

  constructor() {
    super();
    this.name = "World";
  }

  createRenderRoot() {
    return this;
  }

  render() {
    return html`<div class="flex flex-col h-screen">
        <yo-navbar></yo-navbar>
        <yo-cviewer .visible = ${this.cviewerVisibility}></yo-cviewer>
        <div class="grow">
          <yo-cover imagePath = "${cover}" @click = ${() => this.cviewerVisibility = true}></yo-cover>
        </div>
        <div class="sticky bottom-0"><yo-mplayer></yo-mplayer></div>
    </div>`;
  }
}
