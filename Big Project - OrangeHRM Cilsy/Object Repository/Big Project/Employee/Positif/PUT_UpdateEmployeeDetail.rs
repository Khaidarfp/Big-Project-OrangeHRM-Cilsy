<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT_UpdateEmployeeDetail</name>
   <tag></tag>
   <elementGuidId>d1cdc23e-312b-4f81-9848-2291dda8b0dc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;:\&quot;${id}\&quot;,\n    \&quot;firstName\&quot;:\&quot;febri\&quot;,\n    \&quot;middleName\&quot;:\&quot;test\&quot;,\n    \&quot;lastName\&quot;:\&quot;test\&quot;,\n    \&quot;code\&quot;:\&quot;${code}\&quot;,\n    \&quot;dob\&quot;:\&quot;2002-2-2\&quot;,\n    \&quot;lisenceNumber\&quot;:\&quot;A2222\&quot;,\n    \&quot;lisenceNumberExpDate\&quot;:\&quot;2222-2-2\&quot;,\n    \&quot;maritalStatus\&quot;:\&quot;Single\&quot;,\n    \&quot;gender\&quot;:\&quot;M\&quot;,\n    \&quot;otherId\&quot;:null,\n    \&quot;nationality\&quot;:null\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${url}/api/v1/employee/${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>63971ce4-c75d-4b57-86ed-c57b1af0d85a</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>a2305232-fc33-497b-a206-bd663cd3bd5f</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id</defaultValue>
      <description></description>
      <id>c16ab4ea-3ab1-4adc-aa0e-f0047f68b038</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.code</defaultValue>
      <description></description>
      <id>bdba592d-a861-4f31-bbf5-5143a13d87c1</id>
      <masked>false</masked>
      <name>code</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'success', &quot;Successfully Updated&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
