<script>
    import { ListBox, ListBoxItem, RadioGroup, RadioItem } from '@skeletonlabs/skeleton';

    import sawtooth from '$lib/images/sawtooth.svg';
    import sine from '$lib/images/sine.svg';
    import square from '$lib/images/square.svg';
    import triangle from '$lib/images/triangle.svg';
    import minus from '$lib/images/minus.svg';

    let waveform = 'none';

    let waveforms = ['none', 'sawtooth', 'sine', 'square', 'triangle']
    let images = [minus, sawtooth, sine, square, triangle]
</script>

<div class="main card p-4 bg-gradient-to-br from-secondary-500 to-tertiary-500 text-on-primary-token">
    <div class="wave-selection">
        <RadioGroup>
            {#each waveforms as form, i }
                <RadioItem bind:group={ waveform } name={ form } value={ form }><img class:active={ waveform == form } src={ images[i] } /></RadioItem>
            {/each}
        </RadioGroup>
    </div>
    <div class="parameters mt-4">
        <div class="input-group input-group-divider grid-cols-[auto_1fr_auto] pr-4 py-2">
            <div class="input-group-shim bg-transparent">Volume</div>
            <input class="outline-none" type="range" name="volume" value="75" max="100" />
        </div>
    </div>
</div>

<style>
    .main {
        width: fit-content;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .wave-selection {
        font-size: 0.8em;
    }

    .wave-selection img {
        width: 2em;
        height: 2em;
    }

    .active {
        filter: brightness(0) invert(1);
    }
</style>