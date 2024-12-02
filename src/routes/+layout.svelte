<div class="Main_container">
  <slot />
</div>

<style lang="scss">
  :global(html body) {
    padding: 0;
    margin: 0;
  }

  .Main_container {
    color: #ddd;
    // background-color: #f1f1f1;
    background-color: #111;
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100vh;
    // opacity: 0.5;
    position: relative;
    overflow: auto;
    // padding: 20px;
    scrollbar-width: 0;
    &::-webkit-scrollbar {
      display: none;
    }
  }
</style>
