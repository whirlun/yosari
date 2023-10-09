import "./app.css";

import { LitElement, html } from "lit";
import { customElement, property } from "lit/decorators";

@customElement("yo-cover")
class Cover extends LitElement {
  @property({ type: String })
  name = "";
  @property({ type: String })
  imagePath = "";

  createRenderRoot() {
    return this;
  }

  render() {
    return html`
      <div class="stack">
        <div
          class="text-center border border-base-content card w-36 overflow-hidden bg-base-100"
        >
          <div class="card-body p-0">
            <img src="${this.imagePath}" class="object-cover h-full w-full" />
          </div>
        </div>
          <div
            class="text-center border border-base-content card w-36 bg-base-100"
          >
            <div class="card-body">2</div>
          </div>
          <div
            class="text-center border border-base-content card w-36 bg-base-100"
          >
            <div class="card-body">3</div>
          </div>
        </div>
      </div>
    `;
  }
}
