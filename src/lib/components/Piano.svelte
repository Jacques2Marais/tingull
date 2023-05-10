<script>
// @ts-nocheck

    import { invoke } from '@tauri-apps/api/tauri'

    const numWhiteKeys = 7;
    const numBlackKeys = 6;

    let dragging = false;

    function activateKey(e) {
        e.currentTarget.classList.add("activekey");
    }

    function deactivateKey(e) {
        e.currentTarget.classList.remove("activekey");
    }

    function mouseDown(e) {
        dragging = true;

        activateKey(e);
    }

    function mouseUp(e) {
        dragging = false;

        deactivateKey(e);
    }

    function mouseEnter(e) {
        if (dragging) {
            activateKey(e);
        }
    }

    function mouseLeave(e) {
        if (dragging) {
            deactivateKey(e);
        }
    }
</script>

<svelte:document on:mouseup={ mouseUp } />

<div class="main card p-4 bg-gradient-to-br from-secondary-500 to-tertiary-500 text-on-primary-token">
    <div class="whitekeys">
        <div class="octave">
            {#each {length: numWhiteKeys} as _, i}
                <button class="whitekey" on:mousedown={ mouseDown } on:mouseenter={ mouseEnter } 
                    on:mouseup={ mouseUp } on:mouseleave={ mouseLeave }></button>
            {/each}
        </div>
    </div>
    <div class="blackkeys">
        <div class="octave">
            {#each {length: numBlackKeys} as _, i}
                <button class="blackkey" on:mousedown={ mouseDown } on:mouseenter={ mouseEnter } 
                    on:mouseup={ mouseUp } on:mouseleave={ mouseLeave }></button>
            {/each}
        </div>
    </div>
</div>

<style>
    .main {
        width: fit-content;
        height: 100%;
        display: flex;
        flex-direction: row;
        justify-content: center;

        position: relative;
    }

    .whitekey {
        width: 25px;
        height: 100px;
        background-color: white;
        border: 1px solid black;
        border-radius: 5px;
        margin: 0;

        z-index: 1;
    }

    .blackkey {
        width: 15px;
        height: 60px;
        background-color: black;
        border: 1px solid black;
        border-radius: 5px;
        margin: 0 5px;

        z-index: 2;
        pointer-events: all;
    }

    .octave > .blackkey:nth-child(3) {
        visibility: hidden;
    }

    .blackkeys {
        position: absolute;
        top: 16px;
        left: 29px;

        pointer-events: none;
    }

    :global(.whitekey.activekey) {
        background-color: #d9d9d9 !important;
    }

    :global(.blackkey.activekey) {
        background-color: #4c4c4c !important;
    }
</style>