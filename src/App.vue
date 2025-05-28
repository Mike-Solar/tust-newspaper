<script setup lang="ts">
import {ref, shallowRef} from "vue";
import {open, save} from '@tauri-apps/plugin-dialog';
import { invoke } from "@tauri-apps/api/core";
import '@wangeditor/editor/dist/css/style.css' // 引入 css
import { Editor } from '@wangeditor/editor-for-vue'
let isMain=ref(false)
let date_and_num=ref("");
let title=ref("");
let editors=ref("")
const editorConfig = { placeholder: '请输入内容...' }
let editorRefs=Array<any>();
const mode='default'
const handleCreated = (editor:any) => {
  let editorRef=shallowRef();
  editorRef.value=editor;
  editorRefs.push(editorRef) // 记录 editor 实例，重要！
}
const documents = ref(new Array<Document>());
class Document{
  title:string;
  text: string;
  from_who: string
  pictures: Array<any>;
  constructor(){
    this.title = "";
    this.text="";
    this.from_who=""
    this.pictures = [];
  }
}
class Top{
  title: string;
  text: string;

  constructor() {
    this.title="";
    this.text=""
  }

}
const top=ref(new Top());
let showTopDialog=ref(false);
const addDocument=()=>{
  documents.value.push(new Document());
}
const save_to=()=>{
  save({
    filters: [
      {
        name: '便携文档格式（PDF）',
        extensions: ['pdf'],
      },
    ],
  }).then((path)=>{
    for(let i=0;i<documents.value.length;i++){
      documents.value[i].text=editorRefs[i].value.getHtml();
    }
    invoke("save", {
      page:{
        date_and_num: date_and_num.value,
        title: title.value,
        editors: editors.value,
        top:top.value,
        page:documents.value,
        has_top:isMain.value
      },
      path:path
    })
  });
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
const removeArticle=(i:number)=>{
  documents.value.splice(i,1);
  editorRefs.splice(i,1);

}
const removePhoto=(i:number,j:number)=>{
  documents.value[j].pictures.splice(i,1);
}
</script>

<template>
  <main class="container">
    <h1>报刊处理系统</h1>
    <div class="title">
      <el-checkbox class="top" label="是第一版" v-model="isMain"></el-checkbox>
      <label class="top" for="title">标题</label>
      <el-input class="top" id="title" label="标题" v-model="title"></el-input>
      <label class="top" for="editors">编辑</label>
      <el-input class="top" id="editors" label="编辑" v-model="title"></el-input>
    </div>
    <div class="tops" v-if="isMain">
      <label class="top" for="top_title">头版标题</label>
      <el-input class="top" id="top_title" v-model="top.title"/>
      <label class="top" for="top_title">日期</label>
      <el-input class="top" id="top_date" v-model="date_and_num" label="日期" ></el-input>
    </div>
    <div id="top_con">
      <label for="top_title">头版内容</label>
      <el-input id="top_text" v-model="top.text"  label="头版内容" type="textarea" rows="10" />
    </div>
    <div id="documents">
      <div class="articles" v-for="(doc,j) in documents">
        <el-input v-model="doc.title" placeholder="输入文章标题"/>
        <div style="border: 1px solid #ccc">
          <Editor
              style="height: 500px; overflow-y: hidden;"
              v-model="doc.text"
              :defaultConfig="editorConfig"
              :mode="mode"
              @onCreate="handleCreated"
          />
        </div>
        <el-button @click="selectPhotos(doc)" type="default">添加图片</el-button>
        <el-button @click="removeArticle(j)" type="danger">删除</el-button>
        <div style="overflow: scroll">
          <div v-for="(pic,i) in doc.pictures">
            {{pic}}
            <el-button size="small" @click="removePhoto(i,j)" type="danger">删除</el-button>
            <br/>
          </div>
        </div>
      </div>
    </div>
    <div class="button">
      <el-button class="addButton" @click="addDocument" type="default">添加文章</el-button>
      <el-button class="addButton" @click="showTopDialog=true" type="default">编辑头条</el-button>
      <el-button class="addButton" @click="save_to" type="default">保存本版</el-button>
    </div>
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

.button{
  width: 10vw;
  height: 50px;
  bottom: 10vh;
  right: 50vw;
  position: fixed;
  display: flex;
  border: none;
  flex-direction: row;
}
.addButton {
  margin-left: 1vw;
}
#documents {
  width: 80%;
  margin-left: 10vw;
  margin-top: 1vh;
  overflow-x: scroll;
  border: solid 1px;
  border-radius: 5px;
}
#text{
  height: 20vh;
}
.tops{
  width: 80%;
  margin-left: 10%;
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin-top: 1%;
}
.top{
  width: 20vw;
  height: 5vh;
  margin-left: 5%;
}

#top_con{
  width: 80%;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: start;
  margin-left: 10%;
  margin-top: 1%;
}
.title{
  width: 80%;
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-self: center;
  margin-left: 10%;
  margin-top: 1%;
}
.articles{
  width: 80%;
  margin-left: 10%;
  margin-top: 1%;
  margin-bottom: 1%;
}
</style>