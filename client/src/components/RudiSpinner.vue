<template>
<div>
    <p :style='this.style'>
       <b-img src='../assets/img/rudeirer_hat.png' fluid-grow></b-img>
    </p>
</div> 
</template>

<script lang='ts'>
import Vue from 'vue';
export default Vue.extend({
    methods: {
        mouseMoveListener(event: Event) {
            const mouseEvent = event as MouseEvent;
            this.style = 'z-index: 1; ' +
            'position: absolute; ' +
            'left: ' + (mouseEvent.pageX - 256) + 'px; ' +
            'top: ' + (mouseEvent.pageY - 256) + 'px; ' +
            'transform: rotate(-' + this.i + 'deg) ' +
            'scale(' + ((1 + Math.sin(this.blur)) / 2) + ');' +
            'background-color: rgb(' + this.bg + ', ' +
            ((this.bg + 100) % 256) + ', ' + ((this.bg + 200) % 256) + '); ' +
            'filter: hue-rotate(' + 360 * Math.sin(this.blur) + 'deg);';
            this.i = (this.i + 2) % 360;
            this.bg = (this.bg + 1) % 256;
            this.blur = this.blur + 0.05;
            if (this.tm !== 0) {
                window.clearTimeout(this.tm);
                this.tm = 0;
            }
            this.tm = window.setTimeout(() => {
                this.style += ' display: none';
            }, 100);
        },
    },
    mounted() {
        document.addEventListener('mousemove', this.mouseMoveListener);
    },
    beforeDestroy() {
        document.removeEventListener('ousemove', this.mouseMoveListener);
    },
    data() {
        return {
            i: 0,
            bg: 0,
            blur: 0,
            tm: 0,
            style: '',
        };
    },
});
</script>

<style lang='scss' scoped>
</style>


