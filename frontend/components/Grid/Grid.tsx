import axios from 'axios';
import { Dispatch, SetStateAction, useCallback, useEffect, useState } from 'react';
import Alert from '../Alert';

const imageMap = {
    "Costa": "costa.jpg",
    "Rector's Cafe": "rectors.jpg",
    "Taste": "taste.jpg",
    "Main Library": "universityLibrary.jpg"
}

enum Occupancies {
    LOW = "low",
    MEDIUM = "medium",
    HIGH = "high"
}

interface ILocation {
    name: string,
    occupancy: Occupancies,
    time: string,
    date: string
}

const capacityMap = {
    [Occupancies.LOW]: 'Many Seats Available',
    [Occupancies.MEDIUM]: 'Some Seats Available',
    [Occupancies.HIGH]: 'Limited Seats Available'
}

const lowOccupanyStyling = "bg-green-100 text-green-800"
const mediumOccupanyStyling = "bg-amber-100 text-amber-800"
const highOccupanyStyling = "bg-red-100 text-red-800"

interface IGrid {
    setLastUpdated: Dispatch<SetStateAction<Date | undefined>>
}

export default function Grid({ setLastUpdated }: IGrid) {
    const [locations, setLocations] = useState<ILocation[]>()
    // TODO: show errors
    const [error, setError] = useState(false)
    const [showSuccess, setShowSuccess] = useState(false)

    const getUpdatedLocations = useCallback(async () => {
        const { data } = await axios.get("/locations")

        const newData: ILocation[] = data.map((el: { name: string, occupancy: Occupancies, timestamp: number }) => {
            const date = new Date(0)
            date.setUTCSeconds(el.timestamp)
            return ({
                ...el,
                time: date.toLocaleString('en-us', { hour: "numeric" }),
                date: date.toLocaleString('en-us', { year: "numeric", month: "short", day: "numeric" })
            })
        })
        setLocations(newData)
        setLastUpdated(new Date())
    }, [setLastUpdated])

    useEffect(() => {
        getUpdatedLocations()
    }, [getUpdatedLocations])

    async function updateOccupancy(occupancy: Occupancies, name: string) {
        setError(false);
        try {
            await axios.post("/locations", {
                name,
                occupancy
            })
            // update the locations
            getUpdatedLocations()
            setShowSuccess(true)
        }
        catch (error) {
            setError(true);
        }
    }

    return (
        <div>
            <ul role="list" className="grid grid-cols-1 sm:grid-cols-2 gap-6 self-center">
                {locations && locations.map(({ name, occupancy, date, time }) => (
                    <li
                        key={name}
                        className="col-span-1 flex flex-col rounded-lg text-center shadow-xl p-2"
                    >
                        <div className="flex flex-row justify-start gap-4">
                            {/* eslint-disable-next-line @next/next/no-img-element */}
                            <img className="h-32 w-32 rounded-full shadow-md" src={imageMap[name as keyof typeof imageMap]} alt="" />
                            <div className='flex flex-col items-start'>
                                <h3 className="mt-6 text-xl font-semibold text-gray-900">{name}</h3>
                                <dd className="mt-3">
                                    <span className={`${occupancy === Occupancies.LOW && lowOccupanyStyling}
                                                  ${occupancy === Occupancies.MEDIUM && mediumOccupanyStyling}
                                                  ${occupancy === Occupancies.HIGH && highOccupanyStyling}
                                                  rounded-full px-4 py-2 text-md font-medium`}>
                                        {capacityMap[occupancy]}
                                    </span>
                                </dd>
                                <div className='flex flex-col gap-2 mb-2 mt-6 justify-center self-center shadow-md rounded-md'>
                                    <h2 className='text-xs mt-1 text-gray-600 font-medium'>Report Seat Availability</h2>
                                    <div className='flex flex-row justify-center'>
                                        <button
                                            onClick={() => updateOccupancy(Occupancies.LOW, name)}
                                            type="button"
                                            className={`rounded-l-md px-3 py-2 ${lowOccupanyStyling} text-xs  hover:bg-green-50 focus:z-10`}
                                        >
                                            Many
                                        </button>
                                        <button
                                            onClick={() => updateOccupancy(Occupancies.MEDIUM, name)}
                                            type="button"
                                            className={`-ml-px inline-flex items-center px-3 py-2 text-xs ${mediumOccupanyStyling} hover:bg-amber-50 focus:z-10`}
                                        >
                                            Some
                                        </button>
                                        <button
                                            onClick={() => updateOccupancy(Occupancies.HIGH, name)}
                                            type="button"
                                            className={`-ml-px inline-flex items-center rounded-r-md px-3 py-2 text-xs ${highOccupanyStyling} hover:bg-red-50 focus:z-10`}
                                        >
                                            Limited
                                        </button>
                                    </div>
                                </div>

                            </div>
                        </div>
                        <h4 className="mt-2 text-xs font-medium text-gray-600">Last updated: {time}, {date}</h4>
                    </li>
                ))}
            </ul>
            {showSuccess &&
                <div className='z-10 fixed h-full top-[85%] left-1/2 transform -translate-x-1/2'>
                    <Alert setShowSuccess={setShowSuccess} />
                </div>
            }
        </div>
    )
}
