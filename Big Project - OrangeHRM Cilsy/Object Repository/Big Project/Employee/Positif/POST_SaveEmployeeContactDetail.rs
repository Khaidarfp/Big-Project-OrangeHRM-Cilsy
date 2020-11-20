<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST_SaveEmployeeContactDetail</name>
   <tag></tag>
   <elementGuidId>d3358c75-df18-45c5-81dd-efdfde996d63</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;:\&quot;${id}\&quot;,\n    \&quot;addressStreet1\&quot;:null,\n    \&quot;addressStreet2\&quot;:null,\n    \&quot;city\&quot;:\&quot;jakarta\&quot;,\n    \&quot;state\&quot;:\&quot;indonesia\&quot;,\n    \&quot;zip\&quot;:null,\n    \&quot;country\&quot;:null,\n    \&quot;homeTelephone\&quot;:\&quot;08123123\&quot;,\n    \&quot;mobile\&quot;:\&quot;08123123\&quot;,\n    \&quot;workTelephone\&quot;:\&quot;08123123\&quot;,\n    \&quot;workEmail\&quot;:\&quot;${workEmail}\&quot;,\n    \&quot;otherEmail\&quot;:\&quot;${otherEmail}\&quot;\n}&quot;,
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/api/v1/employee/${id}/contact-detail</restUrl>
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
      <id>9badcd0c-4433-43e5-b79b-8b56c6c41f04</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>aabc1d5f-772c-4f2f-af0e-eb99ee2490c6</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id</defaultValue>
      <description></description>
      <id>37897ead-7473-4b98-a951-bbdc159aa152</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.workEmail</defaultValue>
      <description></description>
      <id>3ae53a1e-01b4-4749-b77e-59e0c62361ed</id>
      <masked>false</masked>
      <name>workEmail</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.otherEmail</defaultValue>
      <description></description>
      <id>43ef9b70-05b3-443a-8c26-8aac3cf694ad</id>
      <masked>false</masked>
      <name>otherEmail</name>
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

WS.verifyElementPropertyValue(response, 'success', &quot;Successfully Saved&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
