import { Grid } from '@/components'
import Head from 'next/head'

export default function Home() {

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
          <h2 className='text-gray-400 font-base text-lg mb-20'>Find free spaces, for free. Forever.</h2>
          <Grid/>
        </div>
      </main>
    </>
  )
}
