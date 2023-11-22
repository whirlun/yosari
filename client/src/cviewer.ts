import { LitElement, PropertyValueMap, html } from "lit";
import { customElement, property, state } from "lit/decorators";
import { until } from "lit/directives/until.js";
import cover from "./IMG_0004.jpeg";

@customElement("yo-cviewer")
class ComicViewer extends LitElement {
  @property({ type: Boolean })
  visible = false;

  private totalPages = 9;

  private _leftPage = 2;

  private _rightPage = 1;

  get leftPage() {
    return this._leftPage;
  }

  set leftPage(val) {
    let oldval = this.leftPage;
    if (val < 2) {
      this._leftPage = 2;
    } else if (val > this.totalPages) {
      this._leftPage = oldval;
    } else {
      this._leftPage = val;
    }
    this.requestUpdate("leftPage", oldval);
  }

  get rightPage() {
    return this._rightPage;
  }

  set rightPage(val) {
    let oldval = this.rightPage;
    if (val < 1) {
      this._rightPage = 1;
    } else if (val > this.totalPages) {
      this._rightPage = oldval;
    } else {
      this._rightPage = val;
    }
    this.requestUpdate("rightPage", oldval);
  }

  constructor() {
    super();
    this.visible = false;
  }

  protected createRenderRoot() {
    return this;
  }

  private async getPage(manga_name: string, page: number) {
    return fetch(`http://127.0.0.1:3000/manga/${manga_name}/page/${page}`)
      .then((rep) => rep.blob())
      .then((data) => data);
  }

  private async createURLFromImage(manga_name: string, page: number) {
    if (page === -1) return "";
    let image = await this.getPage(manga_name, page);
    return URL.createObjectURL(image);
  }

  private turnPage(e: KeyboardEvent) {
    if (e.code === "ArrowLeft") {
      this.leftPage += 2;
      this.rightPage += 2;
    } else if (e.code === "ArrowRight") {
      this.leftPage -= 2;
      this.rightPage -= 2;
    }
  }

  protected update(
    changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>
  ) {
    super.update(changedProperties);
    if (this.visible === true) {
      document.getElementById("cviewer").focus();
      this.addEventListener("keydown", this.turnPage);
    } else {
      this.removeEventListener("keydown", this.turnPage);
      this.leftPage = 2;
      this.rightPage = 1;
    }
  }

  render() {
    return html`
      <div
        id="cviewer"
        tabindex="10"
        class="absolute z-10 w-full h-screen flex bg-gray-200 justify-center ${this
          .visible === true
          ? "visible"
          : "invisible"}"
      >
        <img
          src="${until(this.createURLFromImage("onimai", this.leftPage), "")}"
          @click="${() => {
            this.leftPage += 2;
            this.rightPage += 2;
          }}"
        />
        <img
          src="${until(this.createURLFromImage("onimai", this.rightPage), "")}"
          @click="${() => {
            this.leftPage -= 2;
            this.rightPage -= 2;
          }}"
        />
      </div>
    `;
  }
}
