<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT_UpdateEmployeeEducation_Without_SaveData</name>
   <tag></tag>
   <elementGuidId>614861ce-7d60-4848-b94e-28ea65ffd686</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;:\&quot;${id}\&quot;,\n    \&quot;level\&quot;:\&quot;Beginner\&quot;,\n    \&quot;seqId\&quot;:\&quot;1\&quot;,\n    \&quot;institute\&quot;:\&quot;cilsy\&quot;,\n    \&quot;startDate\&quot;:\&quot;2020-2-2\&quot;,\n    \&quot;endDate\&quot;:\&quot;2020-3-3\&quot;,\n    \&quot;specialization\&quot;:\&quot;QA\&quot;,\n    \&quot;year\&quot;:\&quot;2020\&quot;,\n    \&quot;gpa\&quot;:\&quot;4.0\&quot;\n}&quot;,
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
   <restUrl>${url}/api/v1/employee/${id}/education</restUrl>
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
      <id>80b8512e-baee-4b66-a1ef-004f115ad1a9</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>5b99201c-9d46-459e-9232-9f97d843a7e0</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id</defaultValue>
      <description></description>
      <id>a8eb871b-1eee-4f16-b8f7-87db57b7803f</id>
      <masked>false</masked>
      <name>id</name>
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

WS.verifyResponseStatusCode(response, 400)

assertThat(response.getStatusCode()).isEqualTo(400)
WS.verifyElementPropertyValue(response, 'error[0]', 'Employee Education Record Not Found')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
