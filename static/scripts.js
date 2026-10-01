function listenToggleStep() {
  (document.querySelector("#steps") || {}).onclick = (e) => {
    if (e.target.nodeName !== 'LI') {
      return;
    }

    e.target.classList.toggle('done');
  };
}

function listenChangeYield() {
  const target = (document.querySelector("#servings input") || {});
  target.onkeyup = target.onchange = (e) => {
    const defaultAmount = target.dataset.default;
    const currentAmount = parseFloat(target.value);

    if (!isNaN(currentAmount)) {
      const ratio = currentAmount / defaultAmount;
      
      const amountElements = Array.from(document.querySelectorAll(".amount .value"));
      for (const amount of amountElements) {
        const newValue = parseFloat(amount.dataset.default) * ratio;
        amount.innerHTML = Math.round(newValue * 100) / 100;
      }
    }
  };
}

function listenSearch() {
  const target = (document.querySelector("#search") || {});
  const recipes = Array.from(document.querySelectorAll("#recipes li"));

  target.onkeyup = target.onchange = (e) => {
    const searchText = (e.target.value || "").toLowerCase();

    if (searchText) {
      recipes.forEach(r => {
        const recipeName = r.dataset.recipename.toLowerCase();
        const containsSearchText = recipeName.includes(searchText);
        r.classList.toggle("hidden", !containsSearchText);
      });
    } else {
      recipes.map(x => x.classList.toggle('hidden', false));
    }
  };
}

window.onload = () => {
  listenToggleStep();
  listenChangeYield();
  listenSearch();
};