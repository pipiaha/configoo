<template>
    <div>
        configoo
    </div>
</template>

<script>
    export default {
        name: "Home",
        data() {
            return {
                title: ' :)',
            };
        },
        computed: {
            setting() {
                return this.$store.state.app.setting;
            },
        },
        created() {
            this.onLoad();
        },
        methods: {
            onLoad: function () {
                this.$store.dispatch('app/updateModuleValue', {
                    title: this.title,
                    canProceed: () => true,
                });
            },
        }
    }
</script>

<style scoped>

</style>