<template>
<div>
    <b-jumbotron>
        <b-container>
            <RainbowText>Rudeirer News</RainbowText>
            <a class="" href="https://t.me/rudeirer">Offizieller Telegram Channel</a>
        </b-container>
    </b-jumbotron>
    <b-container>
        <NewsItem class="shadow mb-5" v-for="item in news" :key="item.id" :author="item.author" :content="item.content" :date="item.date" :title="item.title"></NewsItem>
    </b-container>
</div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import RainbowText from '@/components/RainbowText.vue';
import NewsItem from '@/components/NewsItem.vue';
@Component({
  components: {
      RainbowText,
      NewsItem,
  },
})
export default class News extends Vue {
    private news = Array.apply(null, Array(5)).map((idx) => ({
        id: idx,
        author: '',
        content: ``,
        date: new Date(),
        title: 'News',
    }));

    private mounted() {
        this.fetchData();
    }
    private async fetchData() {
        const response = await fetch('https://api.rudeirer.wtf/news');
        const news = await response.json();
        if (response.ok && news.success) {
            news.data.forEach((n: any) => n.date = new Date(n.date));
            news.data.sort((a: any , b: any) => b.id - a.id);
            this.news = news.data;
        } else {
            this.$root.$emit('show_alert', 'Error while loading News: ' + news.error);
        }
    }
}
</script>


<style lang="scss" scoped>

</style>
