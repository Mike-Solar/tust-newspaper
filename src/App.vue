<script setup lang="ts">
import { ref } from "vue";
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from "@tauri-apps/api/core";
import '@wangeditor/editor/dist/css/style.css' // 引入 css
import { Editor, Toolbar } from '@wangeditor/editor-for-vue'
let title=ref("");
const documents = ref(new Array<Document>());
class Document{
  title:string;
  text: string;
  pictures: Array<any>;
  constructor(){
    this.title = "";
    this.text="";
    this.pictures = [];
  }
}
const top=ref(new Document());
let showTopDialog=ref(false);
let hasTopDialog=ref(false);
const addDocument=()=>{
  documents.value.push(new Document());
}

function selectPhotos(doc:Document) {
  open({
    multiple: false,
    directory: false,
  }).then((result)=>{
    if(Array.isArray(result)){
      doc.pictures=result;
    }else{
      doc.pictures.push(result);
    }
  });
}
</script>

<template>
  <main class="container">
    <h1>报刊处理系统</h1>
    <div id="documents">
      <el-dialog v-model="showTopDialog">
        <el-input v-model="top.title" placeholder="输入文章标题"/>
        <el-input id="text" v-model="top.text"  placeholder="输入文章内容" type="textarea" rows="80" />
        <el-switch v-model="hasTopDialog" label="显示头条"></el-switch>
        <el-button type="default" @click="showTopDialog=false">确定</el-button>
      </el-dialog>
      <div v-for="doc in documents">
        <el-input v-model="doc.title" placeholder="输入文章标题"/>
        <div style="border: 1px solid #ccc">
          <Toolbar
              style="border-bottom: 1px solid #ccc"
          />
          <Editor
              style="height: 500px; overflow-y: hidden;"
              v-model="doc.text"
          />
        </div>
        <el-button class="button" @click="selectPhotos(doc)" type="default">选择图片</el-button>
        <div style="overflow: scroll">
          <span v-for="pic in doc.pictures">{{pic}}<br></span>
        </div>
      </div>
    </div>
    <el-button id="addButton" class="button" @click="addDocument" type="default">添加文章</el-button>
    <el-button id="addButton" class="button" @click="showTopDialog=true" type="default">编辑头条</el-button>
    <el-button id="addButton" class="button" @click="addDocument" type="default">保存本版</el-button>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
.button{
  width: 10vw;
  height: 50px
}
#addButton {
  position: fixed;
  bottom: 10vh;
  right: 45vw;
}
#documents {
  height: 60vh;
  width: 80vw;
  left: 10vw;
  top:20vh;
  position: absolute;
  overflow-y: scroll;
  overflow-x: scroll;
  border: gray solid 1px;
  border-radius: 5px;
}
#text{
  height: 20vh;
}
</style>