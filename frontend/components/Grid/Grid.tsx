import axios from 'axios';
import { useEffect, useState } from 'react';

const locations = [
    {
        "name": "Taste",
        "occupancy": "low",
        "image": "/taste.jpg",
        "last_update": "2 minutes ago"
    },
    {
        "name": "Costa",
        "occupancy": "low",
        "image": "/costa.jpg",
        "last_update": "10 minutes ago"
    },
    {
        "name": "Rector's Cafe",
        "occupancy": "low",
        "image": "rectors.jpg",
        "last_update": "2 hours ago"
    },
    {
        "name": "Main Library",
        "occupancy": "low",
        "image": "universityLibrary.jpg",
        "last_update": "1 day ago"
    }
]

export default function Grid() {
    // const [locations, setLocations] = useState()

    // const freeSpaceService = axios.create({
    //     baseURL: "https://freespaces.work",
    //     timeout: 3000,
    // });

    // useEffect(() => {
    //   const response = async() => await freeSpaceService.get("/locations")
    //   console.log(response())
    // }, [freeSpaceService])



    return (
        <ul role="list" className="grid grid-cols-1 sm:grid-cols-2 gap-6 self-center">
            {locations.map((location) => (
                <li
                    key={location.name}
                    className="col-span-1 flex flex-col rounded-lg text-center shadow p-2"
                >
                    <div className="flex flex-row justify-start gap-4">
                        {/* eslint-disable-next-line @next/next/no-img-element */}
                        <img className="h-32 w-32 rounded-full" src={location.image} alt="" />
                        <div className='flex flex-col items-start'>
                            <h3 className="mt-6 text-xl font-semibold text-gray-900">{location.name}</h3>
                            <dd className="mt-3">
                                <span className="rounded-full bg-green-100 px-4 py-2 text-md font-medium text-green-800">
                                    {location.occupancy}
                                </span>
                            </dd>
                            <h4 className="mt-2 text-sm font-medium text-gray-600">{location.last_update}</h4>
                        </div>
                    </div>
                    <div className='flex flex-row pb-2 mt-4 justify-center'>
                        <button
                            type="button"
                            className="rounded-l-md px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                        >
                            Low
                        </button>
                        <button
                            type="button"
                            className=" -ml-px inline-flex items-center px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                        >
                            Medium
                        </button>
                        <button
                            type="button"
                            className=" -ml-px inline-flex items-center rounded-r-md px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                        >
                            High
                        </button>
                    </div>

                </li>
            ))}
        </ul>
    )
}
