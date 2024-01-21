import { useEffect, useRef} from 'react'
import gsap from 'gsap-trial'
import DrawSVGPlugin from 'gsap-trial/DrawSVGPlugin'
import LogoS from '../../../assets/images/logo-s.png'
import './index.scss'
import Loader from 'react-loaders'


const Logo = () => {
    const bgRef = useRef<HTMLDivElement>(null)
    const outlineLogoRef = useRef<HTMLDivElement>(null)
    const solidLogoRef = useRef<HTMLImageElement>(null)
    useEffect(() => {
        gsap.registerPlugin(DrawSVGPlugin)

        gsap
            .timeline()
            .to(bgRef.current, {
                duration: 1,
                opacity: 1,
            })
            .from(outlineLogoRef.current, {
                drawSVG: 0,
                duration: 1,
            })

        gsap.fromTo(
            solidLogoRef.current,
            {
                opacity: 0,
            },
            {
                opacity: 1,
                delay: 3.7,
                duration: 4,
            }
        )
    }, [])

    return (
        <>
            <div className="logo-container" ref={bgRef}>
                <img
                    className="solid-logo"
                    ref={solidLogoRef}
                    src={LogoS}
                    alt="JavaScript,  Developer"
                />
            </div>
            <Loader type="pacman" active/>
        </>
    )
}

export default Logo
