<script lang="ts">
  import "../app.css";

  let { children } = $props();
</script>

<header>
  <h1>tachyon</h1>
</header>
<div class="container">
  {@render children()}
</div>
<footer>
  <p>© 2026 tachyon</p>
</footer>
