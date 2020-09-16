<template>
    <div id="app">
        <img alt="Vue logo" src="./assets/logo.png">
        <!--        <input id="b" type="file" webkitdirectory>-->
        <uploader :key="1" :options="options" class="uploader-example"
                  @file-success="onUploadConfirm">
            <uploader-unsupport></uploader-unsupport>
            <uploader-drop>
                <uploader-btn :directory="true" :single="true">选择文件夹</uploader-btn>
            </uploader-drop>
            <uploader-list></uploader-list>
        </uploader>

        <HelloWorld msg="Welcome to Your Vue.js App"/>
    </div>
</template>

<script>
    import HelloWorld from './components/HelloWorld.vue'
    import decoder from './modules/decoder';

    export default {
        name: 'App',
        components: {
            HelloWorld
        },
        data() {
            return {
                options: {
                    // target: '/uploadCategory',//SpringBoot后台接收文件夹数据的接口
                    // testChunks: false//是否分片-不分片
                },

            };
        },
        methods: {
            onUploadConfirm: function (dir, file) {
                // console.log(file);
                if (file.isFolder) {
                    return;
                }
                decoder.decode(file, {});
            }
        }
    }
</script>

<style>
    #app {
        font-family: Avenir, Helvetica, Arial, sans-serif;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-align: center;
        color: #2c3e50;
        margin-top: 60px;
    }
</style>
