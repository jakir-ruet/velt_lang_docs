
    const e = document.getElementsByClassName("item")[0]

    console.log("Hello", document.getElementsByTagName("span")[0])
    let classes = "flex transition-[1s] items-center cursor-pointer text-base pl-3 mb-3 front-normal text-gray-900 rounded-lg dark:text-white hover:text-xl hover:text-cyan-400 hover:font-bold dark:hover:text-cyan-400 text bold: ml-3".split(" ");

    for (let c in classes)
    {
        e.classList.add(c)
    }
