import { Grid } from '@/components'
import Head from 'next/head'
import { useState } from 'react'

export default function Home() {
  const [lastUpdated, setLastUpdated] = useState<Date>()

  return (
    <>
      <Head>
        <title>Freespace</title>
        <meta name="description" content="Find free spaces, for free, forever." />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className='mx-2 my-2'>
        <div className='flex flex-col justify-center items-center'>
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img className="h-16 w-16" src={"/logo.png"} alt="" />
          <h1 className='text-amber-500 font-semibold text-2xl'>Free Space</h1>
          <h2 className='text-gray-400 font-base text-lg mb-4'>Find free spaces, for free. Forever.</h2>
          <h4 className=' text-gray-600 mb-6 max-w-md text-center' >In one of these spaces? Let us know how many seats are available by clicking <span className='text-amber-500'>Many</span>,
            <span className='text-amber-500'> Some</span> or <span className='text-amber-500'>Limited</span>!</h4>
          <Grid setLastUpdated={setLastUpdated} />
        </div>
      </main>
      <footer>
        <div className='text-center'>
          {lastUpdated && <h1 className='text-sm text-gray-600 mb-6 mt-10'>Last updated at {lastUpdated.toLocaleTimeString()}</h1>}
          <h1 className='text-sm text-amber-500 font-medium'>help@freespaces.work</h1>
          <h1 className='text-sm font-medium text-gray-600'>Made with &#129505; by L & F</h1>
        </div>
      </footer>
    </>
  )
}
