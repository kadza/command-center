// entry-point: connect to WebSocket, bind WASD keys for command messages
const ws = new WebSocket('ws://localhost:9001');

ws.addEventListener('open', () => console.log('WebSocket connected'));
ws.addEventListener('message', (event) => console.log('WebSocket received:', event.data));
ws.addEventListener('close', () => console.log('WebSocket closed'));
ws.addEventListener('error', (err) => console.error('WebSocket error:', err));

// Listen for WASD key presses and send as JSON command messages
document.addEventListener('keydown', (e) => {
  const key = e.key.toUpperCase();
  if (['W', 'A', 'S', 'D'].includes(key)) {
    const msg = JSON.stringify({ type: 'cmd', payload: key });
    console.log('WebSocket send:', msg);
    ws.send(msg);
  }
});
