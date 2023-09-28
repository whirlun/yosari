import "./app.css";
import background from "./background.jpeg";

import {html, LitElement} from "lit";
import {customElement, property} from "lit/decorators";


export @customElement("yo-login") class Login extends LitElement {

    constructor() {
        super();
    }

    createRenderRoot() {
        return this; // turn off shadow dom to access external styles
    }

    render() {
        return html`
            <div class="bg-cover min-h-screen" style="background-image: url('${background}')">
                <div class="flex flex-row pt-10">
                <div class="basis-[37.5%]"></div>
                <div class="card content-center bg-base-100/80 basis-1/4 grow">
                    <div class="card-body items-center">
                        <form class="form-control w-full">
                            <label class="label" for="username">
                                <span class="label-text">Username</span>
                            </label>
                            <input type="text" id="username" class="input input-bordered w-full" />
                            <label class="label" for="password">
                                <span class="label-text">Password</span>
                            </label>
                            <input type="password" id="password" class="input input-bordered w-full mb-3" />
                            <input type="submit" class="btn btn-primary w-1/2 place-self-end" value="Login">
                        </form>
                    </div>
                </div>
                <div class="basis-[37.5%]"></div>
            </div>
        </div>`
    }
}