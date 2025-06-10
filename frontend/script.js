fetch("/api/stations")
    .then((res) => res.json())
    .then((stations) => {
        const container = document.getElementById("stations");
        stations.forEach((station) => {
            const div = document.createElement("div");
            div.className = `p-4 rounded-lg shadow-md w-32 text-center text-white font-semibold ${station.reserved ? "bg-red-500" : "bg-green-500"
                }`;
            div.innerText = station.name;
            container.appendChild(div);
        });
    });
