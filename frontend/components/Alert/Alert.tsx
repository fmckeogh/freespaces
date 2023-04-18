import { CheckCircleIcon, XMarkIcon } from '@heroicons/react/20/solid'
import { Dispatch, SetStateAction } from 'react'

interface IAlert {
    setShowSuccess: Dispatch<SetStateAction<boolean>>
}

export default function Alert({setShowSuccess}: IAlert) {
    return (
        <div className="rounded-md bg-green-50 p-4 relative">
            <div className="flex">
                <div className="flex-shrink-0">
                    <CheckCircleIcon className="h-5 w-5 text-green-400" aria-hidden="true" />
                </div>
                <div className="ml-3">
                    <h3 className="text-sm font-medium text-green-800">Report Submitted</h3>
                    <div className="mt-2 text-sm text-green-700">
                        <p>Thank you for contributing to FreeSpaces!</p>
                    </div>
                </div>
                <div className="ml-auto pl-3">
                    <div className="-mx-1.5 -my-1.5">
                        <button
                            onClick={() => setShowSuccess(false)}
                            type="button"
                            className="inline-flex rounded-md bg-green-50 p-1.5 text-green-500 hover:bg-green-100 focus:outline-none"
                        >
                            <span className="sr-only">Dismiss</span>
                            <XMarkIcon className="h-5 w-5" aria-hidden="true" />
                        </button>
                    </div>
                </div>
            </div>
        </div>
    )
}
