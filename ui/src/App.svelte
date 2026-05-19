<script>
  let status = 'Press "New Game" to start.';

  async function newGame() {
    const url = "http://localhost:3000/new-game";
    try {
      const response = await fetch(url, { method: "POST"});
      if (!response.ok) {
        throw new Error(`Response Status: ${response.status}`);
      }

      const result = await response.json();  
      console.log(result);
      status = "Game in Progress..."
    } catch (error) {
      console.error(error.message);
    } 
      
  }
</script>

<main>
  <h1>Ride the Bus</h1>
  <p>{status}</p>
  <button onclick={() => newGame()}>New Game</button>
</main>