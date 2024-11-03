import { useTheme } from '@/provides/AppContext'
import { JsonViewer, defineDataType } from '@textea/json-viewer'
import { useEffect, useState } from 'react'

enum BodyType {
  Text = 'text',
  Json = 'json',
}

export default (props: { body?: string }) => {
  const { body } = props
  const [bodyType, setBodyType] = useState(BodyType.Text)
  const [bodyValue, setBodyValue] = useState(body)

  useEffect(() => {
    try {
      if (body) {
        const value = JSON.parse(body)
        setBodyValue(value)
        setBodyType(BodyType.Json)
      }
    } catch (error) {
      console.log(error)
      setBodyValue(body)
      setBodyType(BodyType.Text)
    }
  }, [body])
  const theme = useTheme()


  if (bodyType == BodyType.Text) {
    return <div>{bodyValue}</div>
  }


  return <JsonViewer theme={theme} rootName={false} value={bodyValue}  />
}
