import { LitElement, html } from "lit";
import { state, customElement, property } from "lit/decorators";
import {when} from "lit/directives/when.js";
import {until} from "lit/directives/until.js";
import {unsafeHTML} from "lit/directives/unsafe-html";
import feather from "feather-icons";

@customElement("yo-mplayer")
class MusicPlayer extends LitElement {
    @state()
    private playing = false;

    @state()
    private currentTime = 0.0;

    @state()
    private duration = 0.0;

    @property({type: String})
    currentMusic = "";

    @property({type: Array<String>})
    musicList = [];

    private audio: HTMLAudioElement = new Audio();

    constructor() {
        super();
    }
    createRenderRoot() {
        return this;
    }

    private async getMusicList():  Promise<string[]> {
        return fetch("http://127.0.0.1:3000/music/list")
        .then(rep => rep.json())
        .then(data => data);
    }

    private playOrPauseMusic() {
        if (this.playing) {
            this.audio.play();
        } else {
            this.audio.pause();
        }
    }

    private switchMusic() {
        if (!this.audio.paused) {
            this.audio.pause();
        }
        this.playing = false;
        this.currentTime = 0.0;
        this.duration = 0.0;
        this.audio = new Audio(`http://127.0.0.1:3000/music/${this.currentMusic}`);
        let getDuration = () => {
            this.audio.currentTime = 0;
            this.audio.removeEventListener('timeupdate', getDuration);
            this.duration = Math.floor(this.audio.duration);
        }
        this.audio.addEventListener("loadedmetadata", (e) => {
            if (this.audio.duration === Infinity) {
                this.audio.currentTime = 1e101;
                this.audio.addEventListener('timeupdate', getDuration);
            }
        });
        this.audio.addEventListener("timeupdate", () => {
            this.currentTime = Math.floor(this.audio.currentTime);
        });
    }

    render() {
        return html`
        <div class="flex justify-center items-center m-2">
            <div button class="btn btn-circle btn-secondary" @click="${() => {this.playing = !this.playing; this.playOrPauseMusic()}}">
                ${when(this.playing, 
                    () => html`${unsafeHTML(feather.icons.pause.toSvg({width: 28, height: 28}))}`, 
                    () => html`${unsafeHTML(feather.icons.play.toSvg({width: 28, height: 28, "style": "margin-left: 4px"}))}`)}
            </div>
            <div class="flex-col">
                <div class="h-3.5 ml-5">${this.currentMusic}</div>
                <div>
                    <progress class="progress progress-primary w-96 ml-5" value="${this.duration === 0 ? 0 : (this.currentTime / this.duration) * 100}" max="100"></progress>
                    <div class="h-3 ml-5">${Math.floor(this.currentTime / 60)}:${this.currentTime % 60}/
                    ${Math.floor(this.duration / 60)}:${this.duration % 60}</div>
                </div>
            </div>
            <div name="playlist" class="dropdown dropdown-top">
                <label tabindex="0" class="btn btn-ghost">
                    ${unsafeHTML(feather.icons.menu.toSvg())}
                </label>
                <ul tabindex="0" class="block dropdown-content menu shadow p-2 bg-base-100 rounded-box h-80 w-52 overflow-x-clip overflow-y-auto">
                    ${until(this.getMusicList().then((l) => l.map((elem) => html`<li><a name=${elem} @click="${(e) => {this.currentMusic = e.target.getAttribute('name');this.switchMusic()}}">${elem}</a></li>`)))}
                </ul>
            </div>
        </div>
        `
    }
}