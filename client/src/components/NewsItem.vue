<template>
    <b-card 
      :title="title"
      class="text-left"
      :footer="footerText()"
      footer-class="text-right">
        <b-card-text>
            <vue-markdown>
{{content}}
            </vue-markdown>
        </b-card-text>
    </b-card>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import VueMarkdown from 'vue-markdown';

@Component({
    components: {
        VueMarkdown,
    },
})
export default class NewsItem extends Vue {
    @Prop(String) private author!: string;
    @Prop(String) private content!: string;
    @Prop(Date) private date!: Date;
    @Prop(String) private title!: string;

    public mounted() {
        const imgs = this.$el.getElementsByTagName('img');
        for (const img of imgs) {
            img.classList.add('img-fluid');
            img.classList.add('mx-auto');
            img.classList.add('d-block');
            img.classList.add('news-img');
        }
    }

    private displayDate() {
        return this.date.toLocaleDateString('de-DE', {
            hour: undefined,
            minute: undefined,
            second: undefined,
        });
    }

    private footerText() {
        return `${this.author || 'unbekannt'} am ${this.displayDate()}`;
    }
}

</script>

<style scoped lang="scss">
.news-img {
    width: 80%;
}

</style>
