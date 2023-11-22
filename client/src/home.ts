import "./app.css";
import cover from "./IMG_0004.jpeg";

import { LitElement, css, html } from "lit";
import { property, customElement, state } from "lit/decorators";
import feather from "feather-icons";
import "./navbar";
import "./cover";
import "./mplayer";
import "./cviewer";
export
@customElement("yo-home")
class Home extends LitElement {
  @property({ type: String })
  name = "";
  static styles = css`
    :host {
      color: blue;
    }
  `;
  @state()
  cviewerVisibility: boolean = false;

  constructor() {
    super();
    this.name = "World";
  }

  createRenderRoot() {
    return this;
  }

  connectedCallback(): void {
    super.connectedCallback();
    window.addEventListener("keydown", (e: KeyboardEvent) => {
      if (this.cviewerVisibility === true && e.code === "Escape") {
        this.cviewerVisibility = false;
      }
    });
  }

  render() {
    return html`
    <div class="flex flex-col h-screen">
      <div class="grow">
        <yo-cviewer .visible=${this.cviewerVisibility}></yo-cviewer>
        <yo-navbar .cviewerVisibility=${this.cviewerVisibility}></yo-navbar>
        <yo-cover
          imagePath="${cover}"
          @click=${() => (this.cviewerVisibility = true)}
        ></yo-cover>
      </div>
      <div class="sticky bottom-0"><yo-mplayer></yo-mplayer></div>
    </div>`;
  }
}
