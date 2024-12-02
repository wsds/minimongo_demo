<script lang="ts">
  function test1() {
    console.log("test1");
    history.go(-1);
  }
</script>

<div class="mian_container">
  <div class="title">
    <div>MiniMongo测试后台</div>
    <a href="" onclick={history.go(-1)}>返回</a>
  </div>

  <div class="testcases_block">
    MiniMongo测试后台
    <div class="case_list">
      <a href="./testcases01/test3"> 最简测试 </a>
    </div>
  </div>
  <div class="testcases_block">MiniMongo数据库管理工具
    <div class="case_list">
      <a href="./testcases01/test3"> 数据管理 </a>
    </div>
  </div>
</div>

<style lang="scss">
  .title {
    display: flex;
    justify-content: space-between;
  }
  .mian_container {
    // background-color: #f1f1f1;
    // background-color: #eef3fa;
    // background-color: #d1d3da;
    display: flex;
    flex-direction: column;
    margin: 20px;

    // width: 100%;
    height: 100%;
    // opacity: 0.5;
    position: relative;
    overflow: auto;
    .case_list {
      margin: 20px 0;
    }
    a {
      color: #fff;
    }
  }

  .testcases_block {
    border: 1px solid #333;
    margin-top: 20px;
    display: flex;
    flex-direction: column;
    padding: 20px;
  }
</style>
