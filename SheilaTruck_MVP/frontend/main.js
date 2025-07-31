
fetch('/fretes')
  .then(res => res.json())
  .then(data => {
    document.getElementById('fretes').innerHTML =
      data.map(f => `<p>${f.origem} → ${f.destino}: R$${f.valor}</p>`).join('');
  });
