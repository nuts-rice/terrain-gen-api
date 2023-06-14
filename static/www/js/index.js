const canvas = document.getElementById('drawing');
const ctx = canvas.getContext('2d');
const exponentInput = document.getElementById('exponent');
const spreadRateInput = document.getElementById('spread_rate');
const renderBtn = document.getElementById('render');
renderBtn.addEventListener('click', () => {
  const exponent = parseInt(exponentInput) || 0;
  const spread_rate = parseFloat(spreadRateInput) || 0;
  
});




