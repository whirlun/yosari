import { LitElement, html } from "lit";
import { customElement, property } from "lit/decorators";
import feather from "feather-icons";
import { unsafeHTML } from "lit/directives/unsafe-html";
@customElement("yo-uploader")
export class MangaUploader extends LitElement {
  @property({ type: Boolean })
  cviewerVisibility = false;

  connectedCallback(): void {
    super.connectedCallback();
    window.addEventListener("dragenter", (e) => {
      if (this.cviewerVisibility) return;
      e.preventDefault();
      this.showDragdrop();
    });
  }

  protected createRenderRoot(): Element | ShadowRoot {
    return this;
  }

  private postFile(files: Array<File>) {
    const form_data = new FormData();
    const produceResult = (a: [string]) => {
      let r = "";
      a.sort();
      for (let s of a) {
        r += `<div class="text-center border border-base-content card w-36 overflow-hidden bg-base-100 rounded-none mx-2">
        <div class="card-body p-0">
          <img src="http://127.0.0.1:3000/upload/${s}" class="object-cover h-full w-full"/>
        </div>
      </div>`;
      }
      return r;
    };
    for (let f of files) {
      form_data.append(f.name, f);
    }
    let fetch_result = fetch("http://127.0.0.1:3000/manga/upload", {
      method: "POST",
      body: form_data,
    });
    // let finish_upload_content = document.getElementById(
    //   "finished"
    // );
    // let loading_content = document.getElementById("uploading");
    // let finish_upload: any = document.getElementById("finish_upload");
    // loading_content.classList.add("flex");
    // loading_content.classList.remove("hidden");
    // finish_upload_content.classList.remove("flex");
    // finish_upload_content.classList.add("hidden");
    // finish_upload.showModal();
    // fetch_result
    //   .then((res) => res.json())
    //   .then((data) => {
    //     loading_content.classList.add("hidden");
    //     loading_content.classList.remove("flex");
    //     finish_upload_content.classList.remove("hidden");
    //     finish_upload_content.classList.add("flex");
    //     finish_upload.showModal();
    //     console.log(data);
    //     sessionStorage.setItem("upload-key", data.msg);
    //     finish_upload_content.innerHTML = `
    //       ${
    //         data.images.length == 0
    //           ? ""
    //           : `
    //       <div class="collapse bg-base-200">
    //       <input type="radio" name="my-accordion-1" checked="checked" /> 
    //         <div class="collapse-title text-xl font-medium">
    //           Images
    //         </div>
    //         <div class="collapse-content flex">
    //           ${produceResult(data.images)}
    //         </div>
    //       </div>
    //       `
    //       }`;
    //   });
  }

  private finishUploadModal() {}

  private showDragdrop() {
    let dragdrop = document.getElementById("dragdrop");
    dragdrop.classList.remove("hidden");
    dragdrop.classList.add("flex");
  }

  private hideDragdrop() {
    let dragdrop = document.getElementById("dragdrop");
    dragdrop.classList.remove("flex");
    dragdrop.classList.add("hidden");
  }

  private handleDrag(e: DragEvent) {
    e.preventDefault();
    this.hideDragdrop();
    this.postFile(Array.from(e.dataTransfer.files));
  }

  protected render() {
    return html`
      <div
        id="dragdrop"
        class="justify-center items-center
             w-full h-full hidden absolute 
             box-border z-50 left-0 top-0
              bg-gray-300/25"
        @dragenter=${(e: DragEvent) => {
          console.log("enter dragzone");
          e.dataTransfer.dropEffect = "copy";
          this.showDragdrop();
          e.preventDefault();
        }}
        @dragover=${(e: DragEvent) => {
          console.log("enter dragzone");
          e.dataTransfer.dropEffect = "copy";
          e.preventDefault();
        }}
        @dragleave=${(e: DragEvent) => {
          this.hideDragdrop();
          e.preventDefault();
        }}
        @drop=${(e: DragEvent) => this.handleDrag(e)}
      >
        <div class="flex-col">
          ${unsafeHTML(
            feather.icons.upload.toSvg({ width: "128px", height: "128px" })
          )}
          <p class="text-lg font-bold">Drag File Here</p>
        </div>
      </div>
      <!-- <dialog id="finish_upload" class="modal">
        <div class="modal-box w-11/12 h-4/6 max-w-5xl">
          <form method="dialog">
            <button
              class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
            >
              ✕
            </button>
          </form>
          <div
            class="flex-col py-4 flex justify-center items-center"
            id="uploading"
          >
            <div>
              ${unsafeHTML(
                feather.icons.loader.toSvg({
                  width: "32px",
                  height: "32px",
                  class: "animate-spin",
                })
              )}
            </div>
            <div class="font-bold text-lg">Uploading</div>
          </div>
          <div class="hidden" id="finished">
          </div>
        </div>
      </dialog> -->

      <div class="flex">
        <button
          class="btn btn-circle btn-ghost"
          @click=${(e) => {
            let input = document.createElement("input");
            input.type = "file";
            input.onchange = () => {
              let files = Array.from(input.files);
              this.postFile(files);
            };
            input.click();
          }}
        >
          ${unsafeHTML(
            feather.icons.upload.toSvg({ width: "28px", height: "28px" })
          )}
        </button>
      </div>
    `;
  }
}
